import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

WebUI.callTestCase(findTestCase('Reclutador/crear vacante/9.- publicar'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Candidato/postulacion/crear postulacion Aforo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.callTestCase(findTestCase('abrir el explorador'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.takeScreenshot('/Users/huguito/Desktop/notificaciones/Reclutador/primeraPostulacion.png')

WebUI.delay(5)

WebUI.closeBrowser()

WebUI.delay(1)

WebUI.callTestCase(findTestCase('Candidato/postulacion/crear postulacion Aforo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.callTestCase(findTestCase('Candidato/postulacion/crear postulacion Aforo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.callTestCase(findTestCase('Candidato/postulacion/crear postulacion Aforo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.callTestCase(findTestCase('Candidato/postulacion/crear postulacion Aforo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.callTestCase(findTestCase('Candidato/postulacion/crear postulacion Aforo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.callTestCase(findTestCase('abrir el explorador'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.takeScreenshot('/Users/huguito/Desktop/notificaciones/Reclutador/sextaPostulacion.png')

WebUI.delay(5)

WebUI.closeBrowser()

WebUI.delay(1)

WebUI.callTestCase(findTestCase('Candidato/postulacion/crear postulacion Aforo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.callTestCase(findTestCase('Candidato/postulacion/crear postulacion Aforo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.callTestCase(findTestCase('Candidato/postulacion/crear postulacion Aforo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.callTestCase(findTestCase('Candidato/postulacion/crear postulacion Aforo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.callTestCase(findTestCase('Candidato/postulacion/crear postulacion Aforo'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.callTestCase(findTestCase('abrir el explorador'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.takeScreenshot('/Users/huguito/Desktop/notificaciones/Reclutador/doceabaPostulacion.png')

WebUI.delay(5)

WebUI.closeBrowser()

