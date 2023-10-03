import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import java.nio.file.WatchService as WatchService
import javax.xml.bind.annotation.XmlElementDecl.GLOBAL as GLOBAL
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.keyword.builtin.GetResponseStatusCodeKeyword as GetResponseStatusCodeKeyword
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import com.kms.katalon.entity.global.GlobalVariableEntity as GlobalVariableEntity
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonOutput as JsonOutput
import groovy.json.JsonSlurper as JsonSlurper
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject

rn = 0

WebUI.callTestCase(findTestCase('Reclutador/login/login'), [:], FailureHandling.STOP_ON_FAILURE)

vacantId = []

response = WS.sendRequest(findTestObject('Reclutador/vacantes/activas/vacantes activas'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

responseText = response.getResponseText()

println(responseText)

def json = new JsonSlurper().parseText(responseText)

json = json.content.vacant.vacantId

vacantId = json.init()

println(json)

vacantId = json

println(vacantId)

RN = json.size()

println(RN)

aleatoreo()

println(rn)

GlobalVariable.vacantId = (vacantId[rn])

println(GlobalVariable.vacantId)

def aleatoreo() {
    rn = ((Math.random() * RN) as int)

    println(rn)
}

